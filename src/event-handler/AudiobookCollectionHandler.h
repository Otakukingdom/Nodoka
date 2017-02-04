//
// Created by mistlight on 2017/02/04.
//

#ifndef NODOKANATIVE_AUDIOBOOKCOLLECTIONHANDLER_H
#define NODOKANATIVE_AUDIOBOOKCOLLECTIONHANDLER_H


#include <src/model/Audiobook.h>

class AudiobookCollectionHandler {

    Audiobook* audiobookModel;
    std::shared_ptr<ProxyManager> manager;

public:
    void directoryAdded(QString path);
    AudiobookCollectionHandler(Audiobook* audiobookModel,
                               std::shared_ptr<ProxyManager> manager);
};


#endif //NODOKANATIVE_AUDIOBOOKCOLLECTIONHANDLER_H
