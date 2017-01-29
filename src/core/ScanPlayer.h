//
// Created by mistlight on 1/28/2017.
//

#ifndef NODOKANATIVE_SCANPLAYER_H
#define NODOKANATIVE_SCANPLAYER_H

#include <QThreadPool>
#include <QMutex>
#include <queue>
#include <memory>
#include <src/proxy-objects/AudiobookProxy.h>
#include <src/proxy-objects/AudiobookFileProxy.h>
#include <include/vlc/vlc.h>

namespace Core {
    class ScanPlayer {
    private:
        QThreadPool scanThread;
        QMutex mutex;
        std::queue<std::shared_ptr<AudiobookFileProxy>> fileQueue;

        // internal function to start the scan task in another thread
        void startScanTask();

        // stuff we need from libvlc
        libvlc_instance_t* inst;
        libvlc_media_player_t* mediaPlayer;
        libvlc_media_t* mediaItem;

    public:
        ScanPlayer();

        void performScan();
        void addAudiobook(std::shared_ptr<AudiobookProxy> audiobook);
        void addAudiobookFile(std::shared_ptr<AudiobookFileProxy> file);
    };
}



#endif //NODOKANATIVE_SCANPLAYER_H
