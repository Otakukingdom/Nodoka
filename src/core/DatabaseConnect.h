//
// Created by mistlight on 1/1/2017.
//

#ifndef NODOKANATIVE_DATABASECONNECT_H
#define NODOKANATIVE_DATABASECONNECT_H

#include <Qt>
#include <QMutexLocker>
#include <libs/lmdb++.h>

namespace Core {
    // Open a connection to the database, if it fails, it will return false
    bool openDb();

    class DatabaseInstance {
        QString dbFilePath;

    public:
        lmdb::env getDbEnv();

        DatabaseInstance();
        ~DatabaseInstance();
    };
}

#endif //NODOKANATIVE_DATABASECONNECT_H
