//
// Created by mistlight on 2017/02/04.
//

#include "AudiobookCollectionHandler.h"

AudiobookCollectionHandler::AudiobookCollectionHandler(Audiobook *audiobookModel,
                                                       std::shared_ptr<ProxyManager> manager) {
    this->manager = manager;
    this->audiobookModel = audiobookModel;
}

void AudiobookCollectionHandler::directoryAdded(QString path) {
    std::shared_ptr<QDir> directoryObject(new QDir(path));

    if(directoryObject->isReadable()) {
        this->audiobookModel->registerAudiobook(QSqlRecord(), directoryObject);
    }
}

void AudiobookCollectionHandler::resetAllReadStatus() {
}
