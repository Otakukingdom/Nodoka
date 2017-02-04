//
// Created by mistlight on 2017/02/04.
//

#include "AudiobookCollectionHandler.h"

AudiobookCollectionHandler::AudiobookCollectionHandler(Audiobook *audiobookModel,
                                                       std::shared_ptr<ProxyManager> manager) {
    this->manager = manager;
    this->audiobookModel = audiobookModel;
}
