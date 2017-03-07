//
// Created by mistlight on 3/5/2017.
//

#include <src/core/AudiobookScan.h>
#include "AddRemoveVisitor.h"

AddRemoveVisitor::AddRemoveVisitor(Audiobook *audiobookModel, QDir baseDirectory) {
    this->audiobookModel = audiobookModel;
    this->baseDirectory = baseDirectory;
}

void AddRemoveVisitor::accept(const std::shared_ptr<QDir>& directory) {
    QDirIterator it(*directory, QDirIterator::NoIteratorFlags);
    std::vector<std::shared_ptr<QDir>> loadedDirectories;
    std::vector<std::shared_ptr<QFile>> loadedAudioFiles;

    while(it.hasNext()) {
        QString currentPath = it.next();

        // check if the path is a directory or a file
        auto potentialDir(std::shared_ptr<QDir>(new QDir(currentPath)));
        auto potentialFile(std::shared_ptr<QFile>(new QFile(currentPath)));


        // if it is a directory, then we
        if(potentialDir->exists()) {
            // don't bother with these..
            if(potentialDir->dirName() == "." || potentialDir->dirName() == "..") {
                continue;
            }

            loadedDirectories.push_back(potentialDir);
        } else if(potentialFile->exists()) {
            if(Core::isAudiobookFile(potentialFile, currentPath)) {
                loadedAudioFiles.push_back(potentialFile);
            }
        }
    }


    if(loadedDirectories.size() > 0) {
        // if all of the directories are similar, then simply make a call
        // to register this audiobook right away, if not already registered
        if(Core::checkDirectorysimilarity(loadedDirectories)) {
            this->addRemoveAudiobook(directory, loadedDirectories, loadedAudioFiles);
        } else {
            for(auto &dir : loadedDirectories) {
                this->accept(dir);
            }
        }
    } else {
        if(loadedAudioFiles.size() > 0) {
            this->addRemoveAudiobook(directory, loadedDirectories, loadedAudioFiles);
        }
    }
}

// TODO: placeholder for zip/archive support
void AddRemoveVisitor::accept(const std::shared_ptr<QFile>& file) {
}

void AddRemoveVisitor::accept(const QString directory) {
    QDir currentDirectory(directory);

    if(currentDirectory.exists()) {
        this->accept(directory);
    }
}

void AddRemoveVisitor::addRemoveAudiobook(const std::shared_ptr<QDir>& directory,
                                          const std::vector<std::shared_ptr<QDir>>& subdirectories,
                                          const std::vector<std::shared_ptr<QFile>>& files) {
}
