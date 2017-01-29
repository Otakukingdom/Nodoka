//
// Created by mistlight on 1/29/2017.
//

#ifndef NODOKANATIVE_PLAYERSCANTASK_H
#define NODOKANATIVE_PLAYERSCANTASK_H


#include <src/core/ScanPlayer.h>
#include <QRunnable>

namespace Core {
    class PlayerScanTask : public QRunnable {
        ScanPlayer* player;
        std::shared_ptr<AudiobookProxy> audiobook;

    public:
        PlayerScanTask(ScanPlayer* player, std::shared_ptr<AudiobookProxy> audiobook);

        void run();
    };
}


#endif //NODOKANATIVE_PLAYERSCANTASK_H
