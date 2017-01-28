//
// Created by mistlight on 1/26/2017.
//

#include "ProxyManager.h"
#include <QDebug>
#include <QDebug>

static const int CACHE_SIZE_MAX = 1000;

ProxyManager::ProxyManager(Core::Setting *settings) {
    this->settings = settings;
}


std::shared_ptr<AudiobookProxy> ProxyManager::getAudiobookProxy(QSqlRecord record) {
    auto key = record.value("id").toString();

    if(this->loadedAudiobooks.contains(key)) {
        return this->loadedAudiobooks.value(key);
    } else {
        auto audiobookEntry = std::shared_ptr<AudiobookProxy>(new AudiobookProxy(record, this->settings));
        this->loadedAudiobooks.insert(key, audiobookEntry);

        return audiobookEntry;
    }
}

std::shared_ptr<AudiobookFileProxy> ProxyManager::getAudiobookFileProxy(QSqlRecord record) {

    auto key = record.value("full_path").toString();
    if(this->abFileCache.contains(key)) {
        return this->abFileCache.value(key);
    } else {
        // if we have too many items in the cache, we should clear it
        if(this->abFileCache.size() > CACHE_SIZE_MAX) {
            this->clearCache();
        }

        auto audiobookEntry = std::shared_ptr<AudiobookFileProxy>(new AudiobookFileProxy(record, this->settings));
        this->abFileCache.insert(key, audiobookEntry);

        return audiobookEntry;
    }
}

void ProxyManager::clearCache() {
    this->abFileCache.clear();
}

