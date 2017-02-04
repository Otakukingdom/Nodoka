//
// Created by mistlight on 2017/02/04.
//

#include <src/model/FileDisplayModel.h>
#include "FileListViewHandler.h"

FileListViewHandler::FileListViewHandler(QMainWindow *window,
                                         QListView *fileListView,
                                         std::shared_ptr<ProxyManager> proxyManager,
                                         QObject *parent) : QObject(parent) {
    this->window = window;
    this->fileListView = fileListView;
    this->manager = proxyManager;
}

void FileListViewHandler::contextMenuRequested(const QPoint &position) {
    // we first want to check if user clicked on an audiobook item
    auto modelIndex = this->fileListView->indexAt(position);
    if(modelIndex.isValid()) {
        // load the file object
        auto model = dynamic_cast<FileDisplayModel*>(this->fileListView->model());
        auto record = model->record(modelIndex.row());
        auto fileProxy = this->manager->getAudiobookFileProxy(record);

        QAction *resetAction = new QAction("Reset Read Status");
        connect(resetAction, &QAction::triggered, [this, fileProxy]() {
            this->handleReset(fileProxy);
        });

        QAction *markAsReadAction = new QAction("Mark as Read");
        connect(markAsReadAction, &QAction::triggered, [this, fileProxy]() {
            this->handleMarkAsRead(fileProxy);
        });

        QAction *removeAction = new QAction("Remove");
        connect(removeAction, &QAction::triggered, [this, fileProxy]() {
            this->handleRemove(fileProxy);
        });

        QMenu *menu = new QMenu(this->window);
        menu->addAction(resetAction);
        menu->addAction(markAsReadAction);
        menu->addAction(removeAction);
        menu->exec(this->fileListView->mapToGlobal(position));
    }
}

void FileListViewHandler::handleReset(std::shared_ptr<AudiobookFileProxy> file) {
    file->resetReadStatus();
    fileListView->update();
}

void FileListViewHandler::handleMarkAsRead(std::shared_ptr<AudiobookFileProxy> file) {
    file->markAsRead();
    fileListView->update();
}

void FileListViewHandler::handleRemove(std::shared_ptr<AudiobookFileProxy> file) {
    file->remove();

    // update the view
    auto model = static_cast<FileDisplayModel*>(fileListView->model());
    model->select();
    fileListView->update();
}

