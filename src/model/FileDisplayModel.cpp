//
// Created by mistlight on 1/6/2017.
//

#include <QSqlError>
#include <QSqlRecord>
#include <QDebug>
#include "FileDisplayModel.h"

FileDisplayModel::FileDisplayModel(QObject *parent) : QSqlTableModel(parent) {
    this->setTable("audiobook_file");
    this->hasFilter = false;
}

void FileDisplayModel::setSelectedAudiobook(int audiobookId) {
    this->selectedAudiobookId = audiobookId;

    this->hasFilter = true;
    this->setFilter("audiobook_id=\'" + QString::number(audiobookId) + "\'");

    // update the selection
    auto res = this->select();
    if(!res) {
        qWarning() << this->lastError().driverText();
    }

    qDebug() << "select called";
}


void FileDisplayModel::selectedAudiobookUpdated(const QItemSelection &selected, const QItemSelection &deselected) {
    if(selected.size() > 0) {
        qDebug() << "file selection changed";
    }
}

QVariant FileDisplayModel::data(const QModelIndex &index, int role) const {
    if(role == Qt::DisplayRole) {
        return this->record(index.row()).value("name");
    }

    return QSqlTableModel::data(index, role);
}
