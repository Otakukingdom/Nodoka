//
// Created by mistlight on 1/2/2017.
//

#include <QtSql>
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
}

