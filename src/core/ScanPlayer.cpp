//
// Created by mistlight on 1/28/2017.
//

#include <src/core/tasks/PlayerScanTask.h>
#include <QDebug>
#include "ScanPlayer.h"

Core::ScanPlayer::ScanPlayer() {
    /* Load the VLC engine */
    this->inst = libvlc_new(0, NULL);
    if(this->inst == NULL) {
        qWarning() << "ERROR";
        throw "Exception has occured, cannot init vlc engine from Scanning";
    }

    this->mediaPlayer = libvlc_media_player_new(this->inst);
    if(this->mediaPlayer == NULL) {
        qWarning() << "ERROR: Could not create media player in Scanning";
    }
}

void Core::ScanPlayer::addAudiobook(std::shared_ptr<AudiobookProxy> audiobook) {
    auto fileList = audiobook->getFilesForAudiobook();
    this->mutex.lock();
    for(int i = 0; i < fileList.size(); i++) {
        this->fileQueue.push(fileList[i]);
    }
    this->mutex.unlock();

    this->startScanTask(audiobook);
}

void Core::ScanPlayer::addAudiobookFile(std::shared_ptr<AudiobookFileProxy> file) {
    // don't need to call this function on already scanned items
    if(file->getMediaDuration() > 0) {
        return;
    }

    this->mutex.lock();
    this->fileQueue.push(file);
    this->mutex.unlock();

    this->startScanTask();
}

void Core::ScanPlayer::startScanTask(std::shared_ptr<AudiobookProxy> audiobook = nullptr) {
    auto scanTask = new PlayerScanTask(this, audiobook);
    this->scanThread.start(scanTask);
}


void Core::ScanPlayer::performScan() {
    this->mutex.lock();

    while(!this->fileQueue.empty()) {
        std::shared_ptr<AudiobookFileProxy> & element = this->fileQueue.front();

        auto path =  element->path();
        auto currentFile = std::unique_ptr<QFile>(new QFile(path));

        if(!currentFile->open(QIODevice::ReadWrite)) {
            qDebug() << "QFILE FAILED!: " << path;
            return;
        }

        this->mediaItem = libvlc_media_new_fd(this->inst, currentFile->handle());
        if(this->mediaItem == NULL) {
            return;
        }

        libvlc_media_player_set_media(this->mediaPlayer, this->mediaItem);
        libvlc_media_player_play(this->mediaPlayer);
        libvlc_media_player_stop(this->mediaPlayer);

        long long duration = libvlc_media_get_duration(this->mediaItem);
        if(duration == -1) {
            element->setMediaDuration(duration);
        } else {
            qWarning() << "performScan() failed for: " << element->path();
        }

        this->fileQueue.pop();
    }

    this->mutex.unlock();
}

