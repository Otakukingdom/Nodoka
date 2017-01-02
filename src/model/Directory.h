//
// Created by mistlight on 1/1/2017.
//

#ifndef NODOKANATIVE_DIRECTORY_H
#define NODOKANATIVE_DIRECTORY_H


#include <QtSql/QSqlRecord>
#include <QtWidgets/QMessageBox>
#include <QtSql/QSqlError>
#include <QtSql/QSqlField>
#include <QtCore/QDateTime>
#include <QSqlTableModel>

class Directory : public QSqlTableModel {
    Q_OBJECT

public:
    Directory(QObject *parent = 0);

    QSqlRecord getEmptyRecord();
    void addDirectory(QString path);
    void removeDirectory(QModelIndex index);

signals:
    void directoryAdded(QSqlRecord);
    void directoryRemove(QSqlRecord);
};


#endif //NODOKANATIVE_DIRECTORY_H
