//
// Created by mistlight on 1/6/2017.
//

#include <QSqlError>
#include <QSqlRecord>
#include <QDebug>
#include <QSqlIndex>
#include "FileDisplayModel.h"

FileDisplayModel::FileDisplayModel(QObject *parent) : QSqlTableModel(parent) {
    this->setTable("audiobook_file");
    auto key = this->primaryKey();
    key.setName("full_path");
    key.setCursorName("full_path");
    this->setPrimaryKey(key);
    this->hasFilter = false;
}

void FileDisplayModel::setSelectedAudiobook(int audiobookId) {
    this->selectedAudiobookId = audiobookId;

    this->hasFilter = true;
    this->setFilter("audiobook_id=\'" + QString::number(this->selectedAudiobookId) + "\'");

    // update the selection
    auto res = this->select();
    if(!res) {
        qWarning() << this->lastError().driverText();
    }
}

QVariant FileDisplayModel::data(const QModelIndex &index, int role) const {
    if(role == Qt::DisplayRole) {
        auto name = this->record(index.row()).
                value("name").toString();
        auto comepleteness = this->record(index.row()).
                value("completeness").toInt();
        auto completenessString = QString::number(comepleteness);

        QString label = "<div class=\"file-item\"><span class=\"name\">" +
                name + "</span><br />" +
                "<span>" + completenessString +"% Completed </span>" +
                "</div>";

        return label;
    }

    return QSqlTableModel::data(index, role);
}

QModelIndex FileDisplayModel::getFileIndex(QString path) {
    for(int i = 0; i < rowCount(); i++) {
        auto currentRecord = this->record(i);

        // select the current file
        if(currentRecord.value("full_path") == path) {
            auto indexObject = this->index(i, 0);

            return indexObject;
        }
    }

    return QModelIndex();
}

