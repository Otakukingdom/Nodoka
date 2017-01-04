//
// Created by mistlight on 1/2/2017.
//

#include "AudiobookRecord.h"

AudiobookRecord::AudiobookRecord() {
    this->readMode = false;
    this->path = this->value("full_path").toString();
}

AudiobookRecord::AudiobookRecord(QString path, bool readMode) {
    // initialize instance vars
    this->readMode = readMode;
    this->path = path;

    // set up the fields
    this->setup();

    // set up the value for the record
    this->setValues();
}

void AudiobookRecord::setValues() {
    this->setValue("full_path", this->path);
    this->setValue("name", this->calculateName());
}

void AudiobookRecord::setup() {
    QSqlField idField;
    idField.setName("id");
    idField.setType(QVariant::Int);
    this->append(idField);

    QSqlField directoryField;
    directoryField.setName("directory");
    directoryField.setType(QVariant::String);
    this->append(directoryField);

    QSqlField nameField;
    nameField.setName("name");
    nameField.setType(QVariant::String);
    this->append(nameField);

    QSqlField pathField;
    pathField.setName("full_path");
    pathField.setType(QVariant::String);
    this->append(pathField);

    QSqlField completenessField;
    completenessField.setName("completeness");
    completenessField.setType(QVariant::Int);
    this->append(completenessField);

    QSqlField defaultOrderField;
    defaultOrderField.setName("default_order");
    defaultOrderField.setType(QVariant::Int);
    this->append(defaultOrderField);

    QSqlField selectedFileField;
    selectedFileField.setName("selected_file");
    selectedFileField.setType(QVariant::String);
    this->append(selectedFileField);

    QSqlField createdAtField;
    createdAtField.setName("created_at");
    createdAtField.setType(QVariant::DateTime);
    this->append(createdAtField);
}

QString AudiobookRecord::calculateName() {
    QString name = "";
    QDir dir(this->path);
    if(dir.exists()) {
        name = dir.dirName();
    } else {
        // if not exists, just default to the path name
        name = dir.path();
    }

    return name;
}

