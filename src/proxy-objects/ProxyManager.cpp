//
// Created by mistlight on 1/26/2017.
//

#include "ProxyManager.h"
#include <QDebug>

static const int CACHE_SIZE_MAX = 1000;

ProxyManager::ProxyManager(Core::Setting *settings) {
    this->settings = settings;
}

std::shared_ptr<AudiobookFileProxy> ProxyManager::getAudiobookFileProxy(QSqlRecord record) {

    auto key = record.value("full_path").toString();
    if(this->audiobookProxyCache.contains(key)) {
        return this->audiobookProxyCache.value(key);
    } else {
        // if we have too many items in the cache, we should clear it
        if(this->audiobookProxyCache.size() > CACHE_SIZE_MAX) {
            this->clearCache();
        }

        auto audiobookEntry = std::shared_ptr<AudiobookFileProxy>(new AudiobookFileProxy(record, this->settings));
        this->audiobookProxyCache.insert(key, audiobookEntry);

        return audiobookEntry;
    }
}

void ProxyManager::clearCache() {
    this->audiobookProxyCache.clear();
}

