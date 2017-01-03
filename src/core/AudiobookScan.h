#include <QtSql>
#include <src/model/Audiobook.h>
#include <src/model/AudiobookFile.h>
#include "Qt"

/**
 * AudiobookScan is a collection of free functions, all designed to scan a directory and
 * operate on the given audiobook and audiobookFiles
 */
namespace Core {

    // function to scan a directory
    void scanDirectory(QSqlRecord directoryRecord, Audiobook* audiobook);

    bool isAudiobookFile(std::shared_ptr<QFile> file);

    QList<QString> getAllFiles(std::shared_ptr<QDir> directory);
}
