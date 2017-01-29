//
// Created by mistlight on 1/2/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOK_H
#define NODOKANATIVE_AUDIOBOOK_H

#include <QSqlTableModel>
#include <memory>
#include <src/proxy-objects/ProxyManager.h>
#include <src/core/ScanPlayer.h>
#include "AudiobookRecord.h"
#include "AudiobookFile.h"

class Audiobook : public QSqlTableModel {
    AudiobookFile* audiobookFile;
    std::shared_ptr<ProxyManager> proxyManager;
    Core::ScanPlayer* scanPlayer;

public:
    Audiobook(AudiobookFile* audiobookFileModel,
              std::shared_ptr<ProxyManager> proxyManager,
              Core::ScanPlayer *scanPlayer,
              QObject *parent = 0);

    QVariant data(const QModelIndex &index, int role) const;

    void registerAudiobook(QSqlRecord baseDirectoryRecord, std::shared_ptr<QDir> directory);
    void removeAudiobook(QSqlRecord record);
    void removeAudiobookByBase(QString path);
};


#endif //NODOKANATIVE_AUDIOBOOK_H
