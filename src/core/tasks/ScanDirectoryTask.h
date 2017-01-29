//
// Created by mistlight on 1/29/2017.
//

#ifndef NODOKANATIVE_SCANDIRECTORYTASK_H
#define NODOKANATIVE_SCANDIRECTORYTASK_H

#include <QSqlRecord>
#include <src/model/Audiobook.h>

namespace Core {
    class ScanDirectoryTask {
        QSqlRecord record;
        Audiobook* audiobook;

    public:
        ScanDirectoryTask(QSqlRecord directoryRecord, Audiobook* audiobook);

        void run();
    };
}


#endif //NODOKANATIVE_SCANDIRECTORYTASK_H
