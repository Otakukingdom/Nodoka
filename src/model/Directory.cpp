//
// Created by mistlight on 1/1/2017.
//

#include "Directory.h"

Directory::Directory(QObject *parent) : QSqlTableModel(parent) {
    this->setTable("directories");
}

