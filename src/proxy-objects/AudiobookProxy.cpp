//
// Created by mistlight on 1/27/2017.
//

#include "AudiobookProxy.h"
#include <QDebug>
#include <QDir>
#include <QDirIterator>
#include <src/model/AudiobookFile.h>
#include <src/core/AudiobookScan.h>

AudiobookProxy::AudiobookProxy(QSqlRecord record,
                               Core::Setting *settings,
                               std::function<std::shared_ptr<AudiobookFileProxy> (QSqlRecord record)> retrieveFileProxyFunction) {
    this->record = record;
    this->settings = settings;
    this->retrieveFileProxyFunction = retrieveFileProxyFunction;

    auto idValue = record.value("id");
    auto directoryValue = record.value("full_path");

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

    this->getFilesForAudiobook();
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
    // get the current paths
    auto fileList = this->getFilesForAudiobook();
    std::set<QString> currentFilePaths;
    std::for_each(fileList.begin(), fileList.end(), [&currentFilePaths](std::shared_ptr<AudiobookFileProxy> f) {
        currentFilePaths.insert(f->path());
    });

    // get the scanned paths
    QDir dir(this->directory);
    // if we can't even read the directory, we should just skip this altogether
    if(!dir.isReadable()) {
        return;
    }

    std::vector<QString> fileToInsert;
    if(dir.isReadable()) {
        QDirIterator it(dir, QDirIterator::NoIteratorFlags);
        while(it.hasNext()) {
            QString currentPath = it.next();
            QFileInfo currentFileInfo(currentPath);
            if(currentFileInfo.isFile()) {
                std::shared_ptr<QFile> currentFile(new QFile(currentPath));

                if (Core::isAudiobookFile(currentFile)) {
                    // see if we have the file on record
                    auto search = currentFilePaths.find(currentPath);
                    if(search == currentFilePaths.end()) {
                        // we do not have the current file on record, we should insert it
                        fileToInsert.push_back(currentPath);
                    }
                }
            }
        }
    }

    // this will insert new files into the list, if no new files are found, then it will simply rearrange the files
    this->insertFiles(fileToInsert);
}


QAction* AudiobookProxy::getRemoveAction() {
    // we do not need to worry about deallocation since we are assuming something else
    // is going to take ownership of this
    auto removeAction = new QAction("Remove Audiobook", this);
    connect(removeAction, &QAction::triggered, this, &AudiobookProxy::remove);
    return removeAction;
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

            auto &callbackFunctionVector = lookupTableResult->second;
            callbackFunctionVector.push_back(callbackFunction);
        } else {
            // create a function list vector, and insert the callback function
            std::vector<std::function<void ()>> callbackFunctionVector;
            callbackFunctionVector.push_back(callbackFunction);
            std::pair<AudiobookEvent, std::vector<std::function<void ()>>> currentPair(callbackType, callbackFunctionVector);

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

/**
 *
 * @param forced If set, this will force the audiobook to perform rescan
 * @return
 */
std::vector<std::shared_ptr<AudiobookFileProxy>> AudiobookProxy::getFilesForAudiobook(bool forced) {
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
            this->updateTotalDuration();
        });

        fileProxy->setCompletenessUpdateFunction([this]() {
            this->updateCompletionStatus();
        });
    }

    return fileList;
}

void AudiobookProxy::updateTotalDuration() {
    auto funcFileList = this->getFilesForAudiobook();
    std::vector<long long> *durationList = new std::vector<long long>();


    std::for_each(funcFileList.begin(), funcFileList.end(),
                  [durationList](std::shared_ptr<AudiobookFileProxy> currentFile) -> long long {
                      auto currentDuration = currentFile->getMediaDuration();
                      if(currentDuration > 0) {
                          durationList->push_back(currentDuration);
                      }

                      return 0;
                  });

    long long totalDuration = std::accumulate(durationList->begin(), durationList->end(), 0);

    delete durationList;
    this->setDuration(totalDuration);
}

void AudiobookProxy::updateCompletionStatus() {
    auto funcFileList = this->getFilesForAudiobook();
    auto totalProgress = 0;

    for(int i = 0; i < funcFileList.size(); i++) {
        auto currentFile = funcFileList[i];
        totalProgress += currentFile->getCurrentTime();
    }

    double completeness = static_cast<double>(totalProgress) / static_cast<double>(this->getDuration());
    int percentage = (int)round(completeness * 100);

    this->currentFileSetting->setValue("completeness", percentage);
}

int AudiobookProxy::getCompleteness() {
    return this->currentFileSetting->value("completeness").toInt();
}

void AudiobookProxy::resetReadStatus() {
    for(auto &fileProxy: this->getFilesForAudiobook()) {
        fileProxy->resetReadStatus();
    }

    this->updateCompletionStatus();
}

void AudiobookProxy::markAsRead() {
    for(auto &fileProxy: this->getFilesForAudiobook()) {
        fileProxy->markAsRead();
    }

    this->updateCompletionStatus();
}

void AudiobookProxy::insertFiles(std::vector<QString> filePathList) {
    auto fileList = this->getFilesForAudiobook();
    std::vector<QString> currentFilePaths;
    std::for_each(fileList.begin(), fileList.end(), [&currentFilePaths](std::shared_ptr<AudiobookFileProxy> f) {
        currentFilePaths.push_back(f->path());
    });

    currentFilePaths.insert(currentFilePaths.end(), filePathList.begin(), filePathList.end());
    std::sort(currentFilePaths.begin(), currentFilePaths.end());

    for(int i = 0; i < currentFilePaths.size(); i++) {
        int position = i + 1;
        auto currentPath = currentFilePaths[i];
        auto result = std::find(std::begin(filePathList), std::end(filePathList), currentPath);

        auto fileObject = this->getFileForPath(currentPath);

        // if fileObject is a null pointer, it means the record need to be created and inserted into the db
        if(fileObject == nullptr) {
            QString queryString = "INSERT INTO audiobook_file(audiobook_id, position, full_path, name) VALUES(?, ?, ?, ?)";
            QSqlQuery queryObject;
            queryObject.prepare(queryString);
            queryObject.addBindValue(this->id);
            queryObject.addBindValue(position);
            queryObject.addBindValue(currentPath);
            queryObject.addBindValue(QFileInfo(currentPath).fileName());
            queryObject.exec();

        } else {
            // otherwise, we need to update the position
            QString queryString = "UPDATE audiobook_file SET position = ? WHERE full_path = ? AND audiobook_id = ?";
            QSqlQuery queryObject;
            queryObject.prepare(queryString);
            queryObject.addBindValue(position);
            queryObject.addBindValue(currentPath);
            queryObject.addBindValue(this->id);
            queryObject.exec();
        }
    }

    // force this object to update the audiobook listing
    this->getFilesForAudiobook(true);
}

std::shared_ptr<AudiobookFileProxy> AudiobookProxy::getFileForPath(QString path) {
    auto fileList = this->getFilesForAudiobook();
    std::shared_ptr<AudiobookFileProxy> fileContainer = nullptr;

    std::for_each(fileList.begin(), fileList.end(), [&fileContainer, path](std::shared_ptr<AudiobookFileProxy> f) {
        if(f->path() == path) {
            fileContainer = f;
            return;
        }
    });

    return fileContainer;
}


