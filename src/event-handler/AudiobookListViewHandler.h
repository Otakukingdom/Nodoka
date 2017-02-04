//
// Created by mistlight on 1/27/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKLISTVIEWHANDLER_H
#define NODOKANATIVE_AUDIOBOOKLISTVIEWHANDLER_H

#include <QObject>
#include <memory>
#include <src/proxy-objects/AudiobookProxy.h>
#include <src/proxy-objects/ProxyManager.h>
#include <QListView>
#include <QMenu>
#include <QMainWindow>


class AudiobookListViewHandler: public QObject {
    Q_OBJECT

    QMainWindow* mainWindow;
    QListView* audiobookListView;
    std::shared_ptr<ProxyManager> proxyManager;

public:
    AudiobookListViewHandler(QMainWindow* window,
                             QListView *audiobookListView,
                             std::shared_ptr<ProxyManager> proxyManager,
                             QObject* parent = 0
    );

public slots:
    void handleResetAudiobook(std::shared_ptr<AudiobookProxy> audiobook);
    void handleDeleteAudiobook(std::shared_ptr<AudiobookProxy> audiobook);
    void handleMarkAsReadAudiobook(std::shared_ptr<AudiobookProxy> audiobook);
    void handleRescan(std::shared_ptr<AudiobookProxy> audiobook);

    void contextMenuRequested(const QPoint &position);

};


#endif //NODOKANATIVE_AUDIOBOOKLISTVIEWHANDLER_H
