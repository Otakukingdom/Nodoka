//
// Created by mistlight on 1/29/2017.
//

#ifndef NODOKANATIVE_INITIALSCANTASK_H
#define NODOKANATIVE_INITIALSCANTASK_H

#include <QRunnable>
#include <src/core/ScanPlayer.h>
#include <src/proxy-objects/AudiobookProxy.h>

namespace Core {
    class InitialScanTask: public QRunnable {
        ScanPlayer* player;
        std::vector<std::shared_ptr<AudiobookProxy>> audiobookList;

    public:
        InitialScanTask(ScanPlayer *player, std::vector<std::shared_ptr<AudiobookProxy>> list);
        void run();

    };
}


#endif //NODOKANATIVE_INITIALSCANTASK_H
