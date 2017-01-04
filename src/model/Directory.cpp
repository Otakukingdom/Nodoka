//
// Created by mistlight on 1/1/2017.
//

#include "Directory.h"

Directory::Directory(QObject *parent) : QSqlTableModel(parent) {
    this->setTable("directories");
    this->setEditStrategy(EditStrategy::OnManualSubmit);

    this->select();
}


void Directory::addDirectory(QString path) {
    // do not continue on a null or empty path string...
    if(path.isEmpty()) {
        return;
    }

    // validate directory path, this function has side effects and will emit error message
    // on it's own, so we don't have to do anything here
    if(!this->validateDirectoryPath(path)) {
        // if invalid, return
        return;
    }

    auto record = this->getEmptyRecord();

    // create the time string
    auto now = QDateTime::currentDateTimeUtc();

    record.setValue("full_path", path);
    record.setValue("created_at", now);

    this->insertRecord(-1, record);
    auto res = this->submitAll();

    if(!res) {
        auto errorObj = this->lastError();
        QMessageBox *messageBox = new QMessageBox();
        messageBox->critical(0, "Error", "Failed to write to config file: " + path + ", error message is: " + errorObj.driverText());
        return;
    }

    emit directoryAdded(record);
}

void Directory::removeDirectory(QModelIndex index) {
    int row = index.row();
    auto record = this->record(row);

    this->removeRow(row);
    auto res = this->submitAll();

    if(!res) {
        auto errorObj = this->lastError();
        QMessageBox *messageBox = new QMessageBox();
        messageBox->critical(0, "Error", "Failed to write to config file, error message is: " + errorObj.driverText());
        return;
    }

    emit directoryRemove(record);
}

QSqlRecord Directory::getEmptyRecord() {
    QSqlRecord record;

    QSqlField pathField;
    pathField.setName("full_path");
    pathField.setType(QVariant::String);
    record.append(pathField);

    QSqlField createdAtField;
    createdAtField.setName("created_at");
    createdAtField.setType(QVariant::DateTime);
    record.append(createdAtField);

    QSqlField lastScannedField;
    lastScannedField.setName("last_scanned");
    lastScannedField.setType(QVariant::DateTime);
    record.append(lastScannedField);

    return record;
}

bool Directory::validateDirectoryPath(QString path) {
    if(this->directoryIsParent(path)) {
        QMessageBox *messageBox = new QMessageBox();
        messageBox->information(0, "Error", "Failed to add directory, cannot add a directory that is a subdirectory of an existing directory");
        return false;
    }

    return true;
}

// check if the current directory is a child directory of any of the directory
// in the model
bool Directory::directoryIsParent(QString potentialChildPath) {
    int rowCount = this->rowCount();
    for(int i = 0; i < rowCount; i++) {
        QSqlRecord currentRecord = this->record(i);
        QString path = currentRecord.field("full_path").value().toString();

        if(potentialChildPath.startsWith(path)) {
            return true;
        }
    }

    return false;
}

