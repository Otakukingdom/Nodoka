//
// Created by mistlight on 1/27/2017.
//

#include <QDebug>
#include <QtSql/QSqlTableModel>
#include <src/model/Audiobook.h>
#include <src/model/FileDisplayModel.h>
#include "AudiobookListViewHandler.h"

AudiobookListViewHandler::AudiobookListViewHandler(QMainWindow *window,
                                                   QListView *audiobookListView,
                                                   QListView *fileListView,
                                                   std::shared_ptr<ProxyManager> proxyManager,
                                                   QObject *parent
): QObject(parent) {
    this->mainWindow = window;
    this->audiobookListView = audiobookListView;
    this->fileListView = fileListView;
    this->proxyManager = proxyManager;
}


void AudiobookListViewHandler::handleResetAudiobook(std::shared_ptr<AudiobookProxy> audiobook) {
    audiobook->resetReadStatus();
}

void AudiobookListViewHandler::handleDeleteAudiobook(std::shared_ptr<AudiobookProxy> audiobook) {
    auto model = dynamic_cast<Audiobook*>(this->audiobookListView->model());
    model->select();
}

void AudiobookListViewHandler::contextMenuRequested(const QPoint &position) {
    // we first want to check if user clicked on an audiobook item
    auto modelIndex = this->audiobookListView->indexAt(position);
    if(modelIndex.isValid()) {
        auto model = dynamic_cast<Audiobook*>(this->audiobookListView->model());
        auto record = model->record(modelIndex.row());
        auto audiobookProxy = this->proxyManager->getAudiobookProxy(record);

        audiobookProxy->addCallback(AudiobookEvent::Removed,
                                    "viewHandlerRemoved",
                                    [this, audiobookProxy]() {
            this->handleDeleteAudiobook(audiobookProxy);
        });

        auto removeAction = audiobookProxy->getRemoveAction();
        auto resetAction = new QAction("Reset Read State");
        connect(resetAction, &QAction::triggered, [this, audiobookProxy] () {
            this->handleResetAudiobook(audiobookProxy);
        });

        auto markAsReadAction = new QAction("Mark as Read");
        connect(markAsReadAction, &QAction::triggered, [this, audiobookProxy] () {
            this->handleMarkAsReadAudiobook(audiobookProxy);
        });

        auto rescanAction = new QAction("Rescan this Audiobook");
        connect(rescanAction, &QAction::triggered, [this, audiobookProxy] () {
            this->handleRescan(audiobookProxy);
        });

        QMenu *menu = new QMenu(this->mainWindow);
        menu->addAction(resetAction);
        menu->addAction(markAsReadAction);
        menu->addAction(rescanAction);
        menu->addAction(removeAction);
        menu->exec(this->audiobookListView->mapToGlobal(position));


    } else {
        // if the user clicked somewhere other than the audiobook item, there is nothing to show...
        return;
    }
}

void AudiobookListViewHandler::handleMarkAsReadAudiobook(std::shared_ptr<AudiobookProxy> audiobook) {
    audiobook->markAsRead();
}

void AudiobookListViewHandler::handleRescan(std::shared_ptr<AudiobookProxy> audiobook) {
    audiobook->rescan();

    // we should update the audiobook list view as well as the file list view
    auto fileModel = static_cast<FileDisplayModel*>(this->fileListView->model());
    fileModel->select();
    this->fileListView->update();

    auto abModel = static_cast<Audiobook*>(this->audiobookListView->model());
    abModel->select();
    this->audiobookListView->update();
}

