//
// Created by mistlight on 1/2/2017.
//

#include <QtWidgets/QMessageBox>
#include <src/core/AudiobookScan.h>
#include <src/core/tasks/ScanDirectoryTask.h>
#include "DirectoryHandler.h"

DirectoryHandler::DirectoryHandler(Audiobook* audiobookModel, AudiobookFile* audiobookFileModel) {
    this->audiobookModel = audiobookModel;
    this->audiobookFileModel = audiobookFileModel;
}

void DirectoryHandler::handleDirectoryAdded(QSqlRecord record) {
    auto task = new Core::ScanDirectoryTask(record, this->audiobookModel);
    QThreadPool::globalInstance()->start(task);
}



void DirectoryHandler::handleDirectoryRemoved(QSqlRecord record) {
    this->audiobookModel->removeAudiobook(record);
}

void DirectoryHandler::handleDirectoryRescan(QSqlRecord record) {
    auto task = new Core::ScanDirectoryTask(record, this->audiobookModel);
    QThreadPool::globalInstance()->start(task);
}
