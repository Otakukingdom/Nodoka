//
// Created by mistlight on 1/2/2017.
//

#include <QtSql/QSqlRecord>
#include "Audiobook.h"

Audiobook::Audiobook(QObject *parent) : QSqlTableModel(parent) {
    this->setTable("audiobooks");
    this->setEditStrategy(EditStrategy::OnManualSubmit);

    this->select();
}

QSqlRecord Audiobook::getEmptyRecord() {
    QSqlRecord record;
    return record;
}
