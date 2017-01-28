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
    Core::Setting* settings;
    QHash<QString, std::shared_ptr<AudiobookFileProxy>> audiobookProxyCache;


    // we load AudiobookProxy objects on demand


public:
    void clearCache();
    std::shared_ptr<AudiobookFileProxy> getAudiobookFileProxy(QSqlRecord);
    ProxyManager(Core::Setting* settings);
};


#endif //NODOKANATIVE_PROXYMANAGER_H
