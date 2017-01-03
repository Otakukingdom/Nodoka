#include <memory>
#include "AudiobookScan.h"

// helper functions

// do the actual recusrive scan directory
static void performScanDirectory(QSqlRecord directoryRecord, std::shared_ptr<QDir> currentDirectory, Audiobook* audiobook);
static bool isAudiobookFile(std::shared_ptr<QFile> file);
static bool checkDirectorysimilarity(std::vector<std::shared_ptr<QDir>> vector);

void Core::scanDirectory(QSqlRecord directoryRecord, Audiobook* audiobook) {
    QString path = directoryRecord.value("full_path").toString();
    std::shared_ptr<QDir> currentDirectory(new QDir(path));

    if(currentDirectory->exists()) {
        performScanDirectory(directoryRecord, currentDirectory, audiobook);
    }
}

void performScanDirectory(QSqlRecord directoryRecord, std::shared_ptr<QDir> currentDirectory, Audiobook* audiobook) {
    QDirIterator it(*currentDirectory, QDirIterator::NoIteratorFlags);
    std::vector<std::shared_ptr<QDir>> loadedDirectories;
    std::vector<std::shared_ptr<QFile>> loadedAudioFiles;

    while(it.hasNext()) {
        QString currentPath = it.next();

        // check if the path is a directory or a file
        std::shared_ptr<QDir> potentialDir(new QDir(currentPath));
        std::shared_ptr<QFile> potentialFile(new QFile(currentPath));

        // if it is a directory, then we
        if(potentialDir->exists()) {
            loadedDirectories.push_back(potentialDir);
        } else if(potentialFile->exists()) {
            if(isAudiobookFile(potentialFile)) {
                loadedAudioFiles.push_back(potentialFile);
            }
        }

        if(loadedDirectories.size() > 0) {
            // if all of the directories are similar, then simply make a call
            // to register this audiobook right away
            if(checkDirectorysimilarity(loadedDirectories)) {
                audiobook->registerAudiobook(currentDirectory);
            } else {
                for(auto &dir : loadedDirectories) {
                    performScanDirectory(directoryRecord, dir, audiobook);
                }
            }
        } else {
            if(loadedAudioFiles.size() > 0) {
                audiobook->registerAudiobook(currentDirectory);
            }
        }
    }
}

bool checkDirectorysimilarity(std::vector<std::shared_ptr<QDir>> dirList) {
    // base case
    if(1 == dirList.size()) {
        return true;
    }

    // TODO: implement string similarity checks here
    return false;
}

bool isAudiobookFile(std::shared_ptr<QFile> file) {
    // by default, non-existing file is not considered to be an audiobook file
    if(!file->exists()) {
        return false;
    }

    QMimeDatabase db;
    QMimeType type = db.mimeTypeForFile(*file);

    if(type.name().startsWith("audio") || type.name().startsWith("video")) {
        return true;
    } else {
        return false;
    }
}
