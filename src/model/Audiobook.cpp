//
// Created by mistlight on 1/2/2017.
//

#include <QtSql>
#include "Audiobook.h"
#include "AudiobookRecord.h"

Audiobook::Audiobook(QObject *parent) : QSqlTableModel(parent) {
    this->setTable("audiobooks");
    this->setEditStrategy(EditStrategy::OnManualSubmit);

    this->select();
}

void Audiobook::registerAudiobook(std::shared_ptr<QDir> directory) {
    AudiobookRecord record(directory->path(), false);
    this->insertRecord(-1, record);
}

