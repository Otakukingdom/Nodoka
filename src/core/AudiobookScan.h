#include <QtSql/QSqlRecord>
#include <src/model/Audiobook.h>
#include <src/model/AudiobookFile.h>
#include "Qt"

namespace Core {

    // function to scan a directory
    void scanDirectory(QSqlRecord directoryRecord, Audiobook* audiobook, AudiobookFile* audiobookFile);

    void reScanDirectory(QSqlRecord directoryRecord, Audiobook* audiobook, AudiobookFile* audiobookFile);

}
