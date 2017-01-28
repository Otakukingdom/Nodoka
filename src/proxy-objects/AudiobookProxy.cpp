//
// Created by mistlight on 1/27/2017.
//

#include "AudiobookProxy.h"
#include <QDebug>
#include <src/model/AudiobookFile.h>

AudiobookProxy::AudiobookProxy(QSqlRecord record, Core::Setting *settings) {
    this->record = record;
    this->settings = settings;

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
