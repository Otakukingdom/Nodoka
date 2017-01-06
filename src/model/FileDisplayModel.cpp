//
// Created by mistlight on 1/6/2017.
//

#include "FileDisplayModel.h"

FileDisplayModel::FileDisplayModel(QObject *parent) : QSqlTableModel(parent) {
    this->setTable("audiobook_files");
    this->hasFilter = false;
}
