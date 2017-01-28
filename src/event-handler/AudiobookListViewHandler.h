//
// Created by mistlight on 1/27/2017.
//

#ifndef NODOKANATIVE_AUDIOBOOKLISTVIEWHANDLER_H
#define NODOKANATIVE_AUDIOBOOKLISTVIEWHANDLER_H

#include <QObject>
#include <memory>
#include <src/proxy-objects/AudiobookProxy.h>
#include <QListView>
#include <QMenu>
#include <QMainWindow>


class AudiobookListViewHandler: public QObject {
    Q_OBJECT

    QMainWindow* mainWindow;
    QListView* audiobookListView;

public:
    AudiobookListViewHandler(QMainWindow* window, QListView *audiobookListView);

public slots:
    void handleResetAudiobook(std::shared_ptr<AudiobookProxy> audiobook);
    void handleDeleteAudiobook(std::shared_ptr<AudiobookProxy> audiobook);
    void contextMenuRequested(const QPoint &position);

};


#endif //NODOKANATIVE_AUDIOBOOKLISTVIEWHANDLER_H
