//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_DIRECTORYHANDLER_H
#define NODOKANATIVE_DIRECTORYHANDLER_H

#include <QtCore>
#include <QtSql>

class DirectoryHandler : public QObject {
    Q_OBJECT
private:
    Audiobook* audiobookModel;
    AudiobookFile* audiobookFileModel;

public:
    DirectoryHandler(Audiobook* audiobookModel, AudiobookFile* audiobookFileModel);
    void handleDirectoryAdded(QSqlRecord record);
    void handleDirectoryRemoved(QSqlRecord record);
    void handleDirectoryRescan(QSqlRecord record);

};


#endif //NODOKANATIVE_DIRECTORYHANDLER_H
