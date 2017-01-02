//
// Created by mistlight on 1/2/2017.
//

#include "AudiobookFileRecord.h"

AudiobookFileRecord::AudiobookFileRecord() {
    this->readMode = false;
}

AudiobookFileRecord::AudiobookFileRecord(QString path, bool readMode) {
    this->readMode = readMode;
}

void AudiobookFileRecord::setup() {
    QSqlField idField;
    idField.setName("id");
    idField.setType(QVariant::Int);
    this->append(idField);

    QSqlField abIdField;
    abIdField.setName("audiobook_id");
    abIdField.setType(QVariant::Int);

    QSqlField nameField;
    nameField.setName("name");
    nameField.setType(QVariant::String);
    this->append(nameField);

    QSqlField pathField;
    pathField.setName("full_path");
    pathField.setType(QVariant::String);
    this->append(pathField);
}

void AudiobookFileRecord::setValues() {
}
