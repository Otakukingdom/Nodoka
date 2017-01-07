//
// Created by mistlight on 1/6/2017.
//

#ifndef NODOKANATIVE_FILEDISPLAYMODEL_H
#define NODOKANATIVE_FILEDISPLAYMODEL_H


#include <QSqlTableModel>
#include <QtCore/QItemSelection>

class FileDisplayModel : public QSqlTableModel {

public:
    FileDisplayModel(QObject *parent = 0);
    void setSelectedAudiobook(int audiobookId);
    QVariant data(const QModelIndex &index, int role) const;

private:
    bool hasFilter;
    int selectedAudiobookId;


public slots:

};


#endif //NODOKANATIVE_FILEDISPLAYMODEL_H
