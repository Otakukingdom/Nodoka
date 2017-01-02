//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_DIRECTORYHANDLER_H
#define NODOKANATIVE_DIRECTORYHANDLER_H

#include <QtCore>
#include <QtSql>

class DirectoryHandler : public QObject {
    Q_OBJECT

public:
    DirectoryHandler();
    void handleDirectoryAdded(QSqlRecord record);
    void handleDirectoryRemoved(QSqlRecord record);

};


#endif //NODOKANATIVE_DIRECTORYHANDLER_H
