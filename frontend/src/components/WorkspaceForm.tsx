import { Modal, Form, Input, message } from 'antd';
import { createWorkspace } from '../api/workspace';

interface Props {
  open: boolean;
  onClose: () => void;
  onCreated: () => void;
}

export default function WorkspaceForm({ open, onClose, onCreated }: Props) {
  const [form] = Form.useForm();

  const handleOk = async () => {
    const values = await form.validateFields();
    try {
      await createWorkspace(values);
      message.success('创建成功');
      form.resetFields();
      onCreated();
      onClose();
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : '创建失败';
      message.error(msg);
    }
  };

  return (
    <Modal title="新建工作区" open={open} onOk={handleOk} onCancel={onClose}>
      <Form form={form} layout="vertical">
        <Form.Item
          name="name"
          label="名称"
          rules={[{ required: true, message: '请输入名称' }]}
        >
          <Input placeholder="例如：深度学习论文集" />
        </Form.Item>
        <Form.Item name="description" label="描述">
          <Input.TextArea rows={3} placeholder="简要描述此工作区的用途..." />
        </Form.Item>
      </Form>
    </Modal>
  );
}
