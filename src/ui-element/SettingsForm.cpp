//
// Created by mistlight on 12/31/2016.
//

#include <QtWidgets/QFileDialog>
#include <QtWidgets/QMessageBox>
#include "SettingsForm.h"

SettingsForm::SettingsForm(Directory* directoryModel, QWidget *parent) :
  QWidget(parent), ui(new Ui::SettingsForm()) {
    ui->setupUi(this);
    ui->listView->setModel(directoryModel);
    ui->listView->setSelectionMode(QAbstractItemView::SelectionMode::SingleSelection);
    this->directoryModel = directoryModel;
    this->setup();
}

SettingsForm::~SettingsForm() {
    delete ui;
}

void SettingsForm::setup() {
    QFont font = QFont("Cabin", 11, 1);
    this->setFont(font);

    connect(this->ui->addDirectoryButton, &QPushButton::clicked, this, &SettingsForm::performAddDirectory);
    connect(this->ui->removeDirectoryButton, &QPushButton::clicked, this, &SettingsForm::performRemoveDirectory);
    connect(this->ui->closeButton, &QPushButton::clicked, this, &SettingsForm::performClose);
    connect(this->ui->rescanButton, &QPushButton::clicked, this, &SettingsForm::performRescan);
}

void SettingsForm::performAddDirectory() {
    auto dir = QFileDialog::getExistingDirectory(this, "Select Folder", "", QFileDialog::ShowDirsOnly);

    // only perform this when user has actually selected something
    if(!dir.isEmpty()) {
        this->directoryModel->addDirectory(dir);
    }
}

void SettingsForm::performRemoveDirectory() {
    // get the currently selected item
    auto indexes = this->ui->listView->selectionModel()->selectedIndexes();
    if(indexes.size() == 0) {
        QMessageBox *messageBox = new QMessageBox();
        messageBox->critical(0, "Error", "You must select a directory to remove");
    } else {
        QModelIndex index = this->ui->listView->selectionModel()->selectedIndexes().first();
        this->directoryModel->removeDirectory(index);
    }
}

void SettingsForm::performClose() {
    this->close();
}

void SettingsForm::performRescan() {
    auto indexes = this->ui->listView->selectionModel()->selectedIndexes();
    if(indexes.size() == 0) {
        QMessageBox *messageBox = new QMessageBox();
        messageBox->critical(0, "Error", "You must select a directory to rescan");
    } else {
        QModelIndex index = this->ui->listView->selectionModel()->selectedIndexes().first();
        auto directoryRecord = this->directoryModel->record(index.row());
        emit this->directoryModel->directoryRescan(directoryRecord);
    }
}
