//
// Created by mistlight on 2017/02/04.
//

#ifndef NODOKANATIVE_FILELISTVIEWHANDLER_H
#define NODOKANATIVE_FILELISTVIEWHANDLER_H


#include <QMainWindow>
#include <QListView>
#include <QMenu>
#include <src/proxy-objects/ProxyManager.h>
#include <memory>

class FileListViewHandler : public QObject {
    Q_OBJECT

private:
    QMainWindow* window;
    QListView* fileListView;
    std::shared_ptr<ProxyManager> manager;

public:
    FileListViewHandler(
        QMainWindow* window,
        QListView *fileListView,
        std::shared_ptr<ProxyManager> proxyManager,
        QObject *parent = 0
    );

    void contextMenuRequested(const QPoint &position);

public slots:
    void handleReset(std::shared_ptr<AudiobookFileProxy> file);
    void handleMarkAsRead(std::shared_ptr<AudiobookFileProxy> file);
};


#endif //NODOKANATIVE_FILELISTVIEWHANDLER_H
