//
// Created by mistlight on 1/26/2017.
//

#ifndef NODOKANATIVE_PROXYMANAGER_H
#define NODOKANATIVE_PROXYMANAGER_H


#include <QSettings>
#include <QSqlRecord>
#include "AudiobookFileProxy.h"
#include "src/proxy-objects/AudiobookProxy.h"


class ProxyManager {
    Core::Setting* settings;
    QHash<QString, std::shared_ptr<AudiobookFileProxy>> audiobookProxyCache;

public:
    void clearCache();
    std::shared_ptr<AudiobookFileProxy> getAudiobookFileProxy(QSqlRecord);
    ProxyManager(Core::Setting* settings);
};


#endif //NODOKANATIVE_PROXYMANAGER_H
