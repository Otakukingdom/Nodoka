//
// Created by mistlight on 1/28/2017.
//

#include <src/core/tasks/PlayerScanTask.h>
#include <QDebug>
#include <include/vlc/vlc.h>
#include <src/simple-lib/ThreadPool.h>
#include <QThread>
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

    this->threadPool = std::unique_ptr<ThreadPool>(new ThreadPool(1));
    this->hasScanFinished = true;
}

void Core::ScanPlayer::addAudiobook(std::shared_ptr<AudiobookProxy> audiobook) {
    qDebug() << "Scan audiobook called";
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

    this->startScanTask(nullptr);
}

void Core::ScanPlayer::startScanTask(std::shared_ptr<AudiobookProxy> audiobook) {
    auto scanTask = new PlayerScanTask(this, audiobook);
    this->scanThread.start(scanTask);
}


void Core::ScanPlayer::performScan() {
    this->mutex.lock();

    qDebug() << "Scan task started";

    while(!this->fileQueue.empty()) {
        this->hasScanFinished = false;
        std::shared_ptr<AudiobookFileProxy> & element = this->fileQueue.front();
        this->currentlyScanning = element;
        qDebug() << "Currently scanning file: " << element->path();

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

        libvlc_event_manager_t* eventManager = libvlc_media_event_manager(this->mediaItem);

        libvlc_event_attach(eventManager,
                            libvlc_MediaStateChanged,
                            (libvlc_callback_t) [](const struct libvlc_event_t * event, void *data) {
                                auto newState = event->u.media_state_changed.new_state;
                                if(newState == libvlc_Playing) {
                                    auto player = reinterpret_cast<ScanPlayer*>(data);

                                    player->threadPool->enqueue([player]() {
                                        long long duration = libvlc_media_get_duration(player->mediaItem);
                                        if(duration == -1) {
                                            player->currentlyScanning->setMediaDuration(duration);
                                        } else {
                                            qWarning() << "performScan() failed for: " << player->currentlyScanning->path();
                                        }


                                        libvlc_media_player_stop(player->mediaPlayer);

                                        player->hasScanFinished = true;
                                        qDebug() << "scan finisehd for file: " << player->currentlyScanning->path();
                                    });
                                }
                            },
                            this);


        while(!hasScanFinished) {
            QThread::msleep(10);
        }

        this->fileQueue.pop();
    }

    this->mutex.unlock();
    qDebug() << "Scan task ended";

}

void Core::ScanPlayer::retrieveScanResults() {

}

