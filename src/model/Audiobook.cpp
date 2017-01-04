//
// Created by mistlight on 1/2/2017.
//

#include <QtSql>
#include <QtWidgets/QMessageBox>
#include "Audiobook.h"

Audiobook::Audiobook(AudiobookFile* audiobookFileModel, QObject *parent) : QSqlTableModel(parent) {
    this->setTable("audiobooks");
    this->setEditStrategy(EditStrategy::OnManualSubmit);

    this->audiobookFile = audiobookFileModel;

    this->select();
}

void Audiobook::registerAudiobook(QSqlRecord baseDirectoryRecord, std::shared_ptr<QDir> directory) {
    AudiobookRecord record(directory->path(), false);
    record.setValue("directory", baseDirectoryRecord.value("full_path").toString());
    record.setValue("completeness", 0);
    record.setValue("default_order", 0);
    record.setNull("selected_file");

    this->insertRecord(-1, record);

    // submit the data
    auto res = this->submitAll();
    if(!res) {
        QMessageBox::critical(0, "Error", "Insert audiobook failed");
    }

    res = this->select();
    if(!res) {
        QMessageBox::critical(0, "Error", "Update select failed");
    }


    int row = getRowForPath(directory->path());
    if(row == -1) {
        QMessageBox::critical(0, "Error", "Audiobook failed to write");
    }

    this->audiobookFile->registerAudioBook(this->record(row).value("id").toInt(), directory);
}

int Audiobook::getRowForPath(QString path) {
    int row = -1;

    for(int i = 0; i < this->rowCount(); i++) {
        auto record = this->record(i);

        if(record.value("full_path").toString() == path) {
            row = i;
            break;
        }
    }

    return row;
}

void Audiobook::removeAudiobook(QString path) {
    int row = this->getRowForPath(path);
    if(row != -1) {
        this->removeAudiobook(row);
    } else {
        QMessageBox::critical(0, "Warning", "Cannot seem to find the audiobook entry to remove...");
    }
}

void Audiobook::removeAudiobook(int row) {
    auto record = this->record(row);
    auto audiobookId = record.value("id").toInt();

    // if we have an empty record.. ignore this
    if(record.isEmpty()) {
        return;
    }

    // remove all the dependencies first
    this->audiobookFile->removeAudiobook(audiobookId);

    // remove the audiobook entry
    this->removeRow(row);
}

void Audiobook::removeAudiobook(QSqlRecord record) {
    this->removeAudiobook(record.value("full_path").toString());
}

