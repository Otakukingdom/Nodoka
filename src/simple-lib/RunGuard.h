//
// Created by mistlight on 1/14/17.
//

#ifndef NODOKANATIVE_RUNGUARD_H
#define NODOKANATIVE_RUNGUARD_H

#include <QObject>
#include <QSharedMemory>
#include <QSystemSemaphore>


class RunGuard
{

public:
    RunGuard( const QString& key );
    ~RunGuard();

    bool isAnotherRunning();
    bool tryToRun();
    void release();

private:
    const QString key;
    const QString memLockKey;
    const QString sharedmemKey;

    QSharedMemory sharedMem;
    QSystemSemaphore memLock;

    Q_DISABLE_COPY( RunGuard )
};


#endif //NODOKANATIVE_RUNGUARD_H
