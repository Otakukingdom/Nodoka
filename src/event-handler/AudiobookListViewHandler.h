//
// Created by mistlight on 1/27/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKLISTVIEWHANDLER_H
#define NODOKANATIVE_AUDIOBOOKLISTVIEWHANDLER_H

#include <QObject>
#include <memory>
#include <src/proxy-objects/AudiobookProxy.h>


class AudiobookListViewHandler: public QObject {
    Q_OBJECT

public:
    AudiobookListViewHandler();

public slots:
    void handleResetAudiobook(std::shared_ptr<AudiobookProxy> audiobook);
    void handleDeleteAudiobook(std::shared_ptr<AudiobookProxy> audiobook);

};


#endif //NODOKANATIVE_AUDIOBOOKLISTVIEWHANDLER_H
