//
// Created by mistlight on 2017/02/04.
//

#ifndef NODOKANATIVE_CHECKSUMTASK_H
#define NODOKANATIVE_CHECKSUMTASK_H

#include <QRunnable>
#include <QThreadPool>
#include <memory>
#include <src/proxy-objects/AudiobookFileProxy.h>

class ChecksumTask : public QRunnable {

    AudiobookFileProxy *audiobookFile;
    bool forced = false;

public:
    static QThreadPool threadPoolInstance;

    ChecksumTask(AudiobookFileProxy*);
    void setForced();

    void run();
};


#endif //NODOKANATIVE_CHECKSUMTASK_H
