//
// Created by mistlight on 1/27/2017.
//

#include "AudiobookProxy.h"
#include <QDebug>
#include <src/model/AudiobookFile.h>

AudiobookProxy::AudiobookProxy(QSqlRecord record,
                               Core::Setting *settings,
                               std::function<std::shared_ptr<AudiobookFileProxy> (QSqlRecord record)> retrieveFileProxyFunction) {
    this->record = record;
    this->settings = settings;
    this->retrieveFileProxyFunction = retrieveFileProxyFunction;

    auto idValue = record.value("id");
    auto directoryValue = record.value("directory");

    if(idValue.isNull() || directoryValue.isNull()) {
        this->isNull = true;
    } else {
        this->isNull = false;

        this->id = idValue.toString();
        this->directory = directoryValue.toString();

        auto stringToHash = "Audiobook:" + this->id + ":" + this->directory;

        auto path = Core::getUniqueSettingPath(stringToHash);
        this->currentFileSetting = QSharedPointer<QSettings>(new QSettings(path, QSettings::IniFormat));
    }
}

void AudiobookProxy::remove() {
    QString queryString = "DELETE FROM audiobooks WHERE id = ?";
    QSqlQuery query;
    query.prepare(queryString);
    query.addBindValue(this->id);
    if(query.exec()) {
        QFile::remove(this->currentFileSetting->fileName());
        int idAsInt = this->id.toInt();
        AudiobookFile::removeAudiobook(idAsInt);

        this->notifyCallbacks(AudiobookEvent::Removed);
    } else {
        qDebug() << "Audiobook Failed to be Removed";
    }
}

void AudiobookProxy::rescan() {

}

QAction* AudiobookProxy::getRemoveAction() {
    auto action = new QAction("Remove Audiobook");
    connect(action, &QAction::triggered, this, &AudiobookProxy::remove);
    return action;
}

void AudiobookProxy::addCallback(AudiobookEvent callbackType,
                                 std::string callbackName,
                                 std::function<void()> callbackFunction) {

    // check if this callback function exists
    auto result = this->callbackFunctionList.find(callbackName);

    // this callback function doesn't exist, time to add it
    if(result == this->callbackFunctionList.end()) {
        auto lookupTableResult = this->callbackLookupTable.find(callbackType);

        if(lookupTableResult != this->callbackLookupTable.end()) {
            // there is already a callback inplace, we have to append the new one
            // at the end of the vector

            qDebug() << "Function inserted";
            auto &callbackFunctionVector = lookupTableResult->second;
            callbackFunctionVector.push_back(callbackFunction);
        } else {
            // create a function list vector, and insert the callback function
            std::vector<std::function<void ()>> callbackFunctionVector;
            callbackFunctionVector.push_back(callbackFunction);
            std::pair<AudiobookEvent, std::vector<std::function<void ()>>> currentPair(callbackType, callbackFunctionVector);

            qDebug() << "Function vector init and inserted";
            this->callbackLookupTable.insert(currentPair);
        }

        // after adding the callback function, time to add it to the set
        // so the same function doesn't get added twice
        this->callbackFunctionList.insert(callbackName);
    }
}

void AudiobookProxy::notifyCallbacks(AudiobookEvent event) {
    auto result = this->callbackLookupTable.find(event);

    if(result != this->callbackLookupTable.end()) {
        auto callbackFunctionList = result->second;
        for(auto &callbackFunction : callbackFunctionList) {
            callbackFunction();
        }
    }
}

std::vector<std::shared_ptr<AudiobookFileProxy>> AudiobookProxy::getFilesForAudiobook() {
    // we place a mutex on here so that this function does NOT get called on multiple times
    // by different threads when the fileListCache is still building at the same time
    this->mutex.lock();

    // look into the cache to see if we already have a record of this
    if(this->fileListCache.size() > 0) {
        this->mutex.unlock();
        return this->fileListCache.toStdVector();
    }

    auto fileList = this->filesForAudiobookByDb(this->id, this->retrieveFileProxyFunction);
    this->mutex.unlock();
    return fileList;
}



long long AudiobookProxy::getDuration() {
    return this->currentFileSetting->value("duration").toLongLong();
}

void AudiobookProxy::setDuration(const long long duration) {
    this->currentFileSetting->setValue("duration", duration);
    this->currentFileSetting->sync();
}

void AudiobookProxy::handlePropertyScanFinished() {
    auto fileList = this->getFilesForAudiobook();
    long long duration = 0;

    for(auto &fileEntry : fileList) {
        duration += fileEntry->getMediaDuration();
    }

    this->setDuration(duration);
}

bool AudiobookProxy::hasDuration() {
    return !this->currentFileSetting->value("duration").isNull();
}

bool AudiobookProxy::allFileDurationScanned() {
    auto fileList = this->getFilesForAudiobook();
    auto scanned = true;
    for(auto &audiobookFile : fileList) {
        if(audiobookFile->getMediaDuration() <= 0) {
            scanned = false;
        }
    }

    return scanned;
}

std::vector<std::shared_ptr<AudiobookFileProxy>> AudiobookProxy::filesForAudiobookByDb(
        QString audiobookId,
        std::function<std::shared_ptr<AudiobookFileProxy> (QSqlRecord record)> retrieveFileProxyFunction) {
    std::vector<std::shared_ptr<AudiobookFileProxy>> fileList;

    QString queryString = "SELECT * FROM audiobook_file WHERE audiobook_id = ?";
    QSqlQuery query;
    query.prepare(queryString);
    query.addBindValue(audiobookId.toInt());
    query.exec();

    while(query.next()) {
        auto record = query.record();
        auto fileProxy = retrieveFileProxyFunction(record);
        fileList.push_back(fileProxy);

        this->fileListCache.push_back(fileProxy);

        fileProxy->setTotalDurationUpdateFunction([this]() {
            auto funcFileList = this->getFilesForAudiobook();
            std::vector<long long> durationList;

            std::transform(funcFileList.begin(), funcFileList.end(), durationList.begin(),
                           [](std::shared_ptr<AudiobookFileProxy> currentFile) -> long long {
                               if(currentFile->getMediaDuration() > 0) {
                                   return currentFile->getMediaDuration();
                               }

                               return 0;
            });

            long long totalDuration = std::accumulate(durationList.begin(), durationList.end(), 0);
            this->setDuration(totalDuration);
        });
    }

    return fileList;
}



