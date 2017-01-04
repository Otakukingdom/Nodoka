//
// Created by mistlight on 1/2/2017.
//

#include <QtWidgets/QMessageBox>
#include <src/core/AudiobookScan.h>
#include "DirectoryHandler.h"

DirectoryHandler::DirectoryHandler(Audiobook* audiobookModel, AudiobookFile* audiobookFileModel) {
    this->audiobookModel = audiobookModel;
    this->audiobookFileModel = audiobookFileModel;
}

void DirectoryHandler::handleDirectoryAdded(QSqlRecord record) {
    Core::scanDirectory(record, this->audiobookModel);
}

void DirectoryHandler::handleDirectoryRemoved(QSqlRecord record) {
    this->audiobookModel->removeAudiobook(record);
}
