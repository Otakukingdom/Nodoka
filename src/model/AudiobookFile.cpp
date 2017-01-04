//
// Created by mistlight on 1/2/2017.
//

#include <QtCore/QDirIterator>
#include <src/core/AudiobookScan.h>
#include <QtWidgets/QMessageBox>
#include "AudiobookFileRecord.h"


AudiobookFile::AudiobookFile(QObject *parent) : QSqlTableModel(parent) {
    this->setTable("audiobook_file");
    this->select();
}

void AudiobookFile::addAudiobookFile(int audiobookId, int position, QString path) {
    // initialize the record
    AudiobookFileRecord record(false);
    record.setValue("audiobook_id", audiobookId);
    record.setValue("position", position);
    record.setValue("full_path", path);
    record.setValue("name", QFileInfo(path).fileName());
    auto inRes = this->insertRecord(-1, record);
    if(!inRes) {
        QMessageBox::critical(0, "Error", "Failed to add: " + this->lastError().driverText() + " with db reason of " + this->lastError().databaseText());
        return;
    }

    auto res = this->submitAll();
    if(!res) {
        QMessageBox::critical(0, "Error", "Failed to add: " + this->lastError().driverText() + " with db reason of " + this->lastError().databaseText());
        return;
    }
}

void AudiobookFile::registerAudioBook(int audiobookId, std::shared_ptr<QDir> directory) {
    QList<QString> filePathList = Core::getAllFiles(directory);

    int position = 1;
    for(auto &currentPath : filePathList) {
        // check if the file isn't already added, if it isn't, then add it
        QSqlQuery query;
        query.prepare("SELECT * FROM audiobook_file WHERE full_path=?");
        query.addBindValue(currentPath);
        auto res = query.exec();

        if(!res) {
            qWarning() << "SELECT check failed because query failed to run";
            continue;
        }

        // if we can't find a result, it means we should add the current audiobook file
        if(!query.next()) {
            this->addAudiobookFile(audiobookId, position, currentPath);
        }

        position++;
    }
}

void AudiobookFile::removeAudiobook(int audiobookId) {
    QSqlQuery query;
    query.prepare("DELETE FROM audiobook_file WHERE audiobook_id = ?");
    query.addBindValue(audiobookId);
    auto res = query.exec();
    if(!res) {
        QMessageBox::critical(0, "Error", "Failed to delete file entry");
    }
}

