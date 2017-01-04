//
// Created by mistlight on 1/2/2017.
//

#include "AudiobookFileRecord.h"

AudiobookFileRecord::AudiobookFileRecord() {
    this->setup();
    this->readMode = true;
}

AudiobookFileRecord::AudiobookFileRecord(bool readMode) {
    this->setup();
    this->readMode = readMode;
    this->setInitValues();
}

void AudiobookFileRecord::setup() {
    QSqlField abIdField;
    abIdField.setName("audiobook_id");
    abIdField.setType(QVariant::Int);
    this->append(abIdField);

    QSqlField nameField;
    nameField.setName("name");
    nameField.setType(QVariant::String);
    this->append(nameField);

    QSqlField pathField;
    pathField.setName("full_path");
    pathField.setType(QVariant::String);
    this->append(pathField);

    QSqlField lengthOfFileField;
    lengthOfFileField.setName("length_of_file");
    lengthOfFileField.setType(QVariant::String);
    this->append(lengthOfFileField);

    QSqlField positionField;
    positionField.setName("position");
    positionField.setType(QVariant::Int);
    this->append(positionField);

    QSqlField seekPositionField;
    seekPositionField.setName("seek_position");
    seekPositionField.setType(QVariant::Double);
    this->append(seekPositionField);

    QSqlField fileExistsField;
    fileExistsField.setName("file_exists");
    fileExistsField.setType(QVariant::Bool);
    this->append(fileExistsField);

    QSqlField createdAtField;
    createdAtField.setName("created_at");
    createdAtField.setType(QVariant::DateTime);
    this->append(createdAtField);
}

void AudiobookFileRecord::setInitValues() {
    this->setValue("file_exists", true);
    this->setValue("seek_position", 0.0);
    this->setValue("completeness", 0);
}
