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

    public:
        PlayerScanTask(ScanPlayer* player);

        void run();
    };
}


#endif //NODOKANATIVE_PLAYERSCANTASK_H
