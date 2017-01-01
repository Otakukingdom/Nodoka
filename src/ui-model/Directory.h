//
// Created by mistlight on 1/1/2017.
//

#ifndef NODOKANATIVE_DIRECTORY_H
#define NODOKANATIVE_DIRECTORY_H


#include <QSqlTableModel>

class Directory : public QSqlTableModel {
    Q_OBJECT

public:
    Directory(QObject *parent = 0);

};


#endif //NODOKANATIVE_DIRECTORY_H
