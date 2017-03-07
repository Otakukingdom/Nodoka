#include <QtSql>
#include <src/model/Audiobook.h>
#include <src/model/AudiobookFile.h>
#include <QRunnable>
#include "Qt"
#include <QProgressDialog>


/**
 * AudiobookScan is a collection of free functions, all designed to scan a directory and
 * operate on the given audiobook and audiobookFiles
 */
namespace Core {

    // function to directory-element-scan a directory
    void scanDirectory(QSqlRecord directoryRecord, Audiobook* audiobook);

    bool isAudiobookFile(std::shared_ptr<QFile> file, QString path = QString());

    bool isAudiobookFile(const QFile& file, QString path = QString());

    QList<QString> getAllFiles(std::shared_ptr<QDir> directory);

    bool checkDirectorysimilarity(std::vector<std::shared_ptr<QDir>> directoryList);
}
