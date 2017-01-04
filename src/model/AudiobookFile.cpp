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
    AudiobookFileRecord record(path, false);
    record.setInitValues();
    record.setValue("audiobook_id", audiobookId);
    record.setValue("position", position);
    this->insertRecord(-1, record);
    this->submitAll();
}

void AudiobookFile::registerAudioBook(int audiobookId, std::shared_ptr<QDir> directory) {
    QList<QString> filePathList = Core::getAllFiles(directory);

    int position = 1;
    for(auto &currentPath : filePathList) {
        // check if the file isn't already added, if it isn't, then add it
        if(getRowForPath(currentPath) == -1) {
            this->addAudiobookFile(audiobookId, position, currentPath);
        }

        position++;
    }
}

int AudiobookFile::getRowForPath(QString path) {
    int row = -1;

    for(int i = 0; i < rowCount(); i++) {
        if(this->record().value("full_path").toString() == path) {
            row = i;
        }
    }

    return row;
}

void AudiobookFile::removeAudiobook(int audiobookId) {
    QSqlQuery query;
    query.prepare("DELETE FROM audiobook_file WHERE audiobook_id = ?");
    query.addBindValue(audiobookId);
    auto res = query.exec();
    if(!res) {
        QMessageBox::critical(0, "Error", "Failed to delete file entry");
    }

    // ensure the changes are final
    this->submitAll();
}

