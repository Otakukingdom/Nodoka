//
// Created by mistlight on 1/6/2017.
//

#ifndef NODOKANATIVE_FILEDISPLAYMODEL_H
#define NODOKANATIVE_FILEDISPLAYMODEL_H


#include <QSqlTableModel>
#include <QtCore/QItemSelection>
#include "ProxyManager.h"

class FileDisplayModel : public QSqlTableModel {

public:
    FileDisplayModel(std::shared_ptr<ProxyManager> manager, QObject *parent = 0);
    void setSelectedAudiobook(int audiobookId);
    QModelIndex getFileIndex(QString path);
    QVariant data(const QModelIndex &index, int role) const;

private:
    std::shared_ptr<ProxyManager> manager;
    bool hasFilter;
    int selectedAudiobookId;


public slots:

};


#endif //NODOKANATIVE_FILEDISPLAYMODEL_H
