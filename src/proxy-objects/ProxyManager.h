//
// Created by mistlight on 1/26/2017.
//

#ifndef NODOKANATIVE_PROXYMANAGER_H
#define NODOKANATIVE_PROXYMANAGER_H


#include <QSettings>
#include <QSqlRecord>
#include "AudiobookFileProxy.h"
#include "src/proxy-objects/AudiobookProxy.h"


/**
 * ProxyManager is responsible for querying various Proxy objects
 *
 * AudiobookProxy is the proxy object that is a representation of an Audiobook Record.
 *
 */
class ProxyManager {
private:
    Core::Setting* settings;

    // keep a record of files
    QHash<QString, std::shared_ptr<AudiobookFileProxy>> abFileCache;

    // stores the current audiobook records
    QHash<QString, std::shared_ptr<AudiobookProxy>> loadedAudiobooks;


public:
    void clearCache();
    std::shared_ptr<AudiobookFileProxy> getAudiobookFileProxy(QSqlRecord);
    std::shared_ptr<AudiobookProxy> getAudiobookProxy(QSqlRecord);
    ProxyManager(Core::Setting* settings);
};


#endif //NODOKANATIVE_PROXYMANAGER_H
