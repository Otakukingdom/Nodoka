//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOK_H
#define NODOKANATIVE_AUDIOBOOK_H

#include <QSqlTableModel>

class Audiobook : public QSqlTableModel {

public:
    Audiobook(QObject *parent = 0);
    QSqlRecord getEmptyRecord();

};


#endif //NODOKANATIVE_AUDIOBOOK_H
